#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Show {
    id: u64,
    title: String,
    genre: String,
    start_time: u64,
    end_time: u64,
    total_tickets: u32,
    available_tickets: u32,
}

impl Storable for Show {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Show {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static SHOW_STORAGE: RefCell<StableBTreeMap<u64, Show, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct ShowPayload {
    title: String,
    genre: String,
    start_time: u64,
    end_time: u64,
    total_tickets: u32,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Booking {
    id: u64,
    show_id: u64,
    user_id: u64,
    num_tickets: u32,
}

impl Storable for Booking {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Booking {
    const MAX_SIZE: u32 = 1024;  // You can adjust this based on your needs
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static BOOKING_STORAGE: RefCell<StableBTreeMap<u64, Booking, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct BookingPayload {
    show_id: u64,
    user_id: u64,
    num_tickets: u32,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    NotEnoughTickets,
    InvalidInput,
}

// a helper method to get a show by id. used in get_show/update_show
fn _get_show(id: &u64) -> Option<Show> {
    SHOW_STORAGE.with(|service| service.borrow().get(id))
}

#[ic_cdk::query]
fn get_show(id: u64) -> Result<Show, Error> {
    match _get_show(&id) {
        Some(show) => Ok(show),
        None => Err(Error::NotFound {
            msg: format!("a show with id={} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn add_show(show: ShowPayload) -> Option<Show> {
    // Validate input data
    if show.title.is_empty() || show.genre.is_empty() || show.total_tickets == 0 {
        return None; // Reject invalid input
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let show = Show {
        id,
        title: show.title,
        genre: show.genre,
        start_time: show.start_time,
        end_time: show.end_time,
        total_tickets: show.total_tickets,
        available_tickets: show.total_tickets, // Initially all tickets are available
    };
    do_insert_show(&show);
    Some(show)
}

#[ic_cdk::update]
fn update_show(id: u64, payload: ShowPayload) -> Result<Show, Error> {
    match SHOW_STORAGE.with(|service| service.borrow().get_mut(&id)) {
        Some(mut show) => {
            update_show_common(&mut show, &payload.total_tickets)?;
            do_insert_show(&show);
            Ok(show.clone())
        }
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't update a show with id={}. show not found",
                id
            ),
        }),
    }
}

// helper method to perform insert.
fn do_insert_show(show: &Show) {
    SHOW_STORAGE.with(|service| service.borrow_mut().insert(show.id, show.clone()));
}

#[ic_cdk::update]
fn delete_show(id: u64) -> Result<Show, Error> {
    match SHOW_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(show) => Ok(show),
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't delete a show with id={}. show not found.",
                id
            ),
        }),
    }
}

#[ic_cdk::query]
fn get_booking(id: u64) -> Result<Booking, Error> {
    match BOOKING_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(booking) => Ok(booking.clone()),
        None => Err(Error::NotFound {
            msg: format!("a booking with id={} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn add_booking(booking: BookingPayload) -> Option<Booking> {
    // Validate input data
    if booking.num_tickets == 0 {
        return None; // Reject invalid input
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let show = _get_show(&booking.show_id);
    if show.is_none() {
        return None; // Show not found, so booking is not allowed
    }

    let booking = Booking {
        id,
        show_id: booking.show_id,
        user_id: booking.user_id,
        num_tickets: booking.num_tickets,
    };

    // Check if there are enough available tickets
    let available_tickets = show.as_ref().unwrap().available_tickets;
    if booking.num_tickets > available_tickets || booking.num_tickets == 0 {
        return None; // Not enough available tickets or invalid number of tickets
    }

    // Update available tickets after booking
    let mut updated_show = show.unwrap().clone(); // Clone to avoid moving
    updated_show.available_tickets -= booking.num_tickets;
    do_insert_show(&updated_show);

    // Insert the booking
    do_insert_booking(&booking);
    Some(booking)
}

#[ic_cdk::update]
fn update_booking(id: u64, payload: BookingPayload) -> Result<Booking, Error> {
    let booking_result = BOOKING_STORAGE.with(|service| service.borrow().get(&id));
    let booking = match booking_result {
        Some(booking) => booking.clone(),
        None => return Err(Error::NotFound {
            msg: format!("a booking with id={} not found", id),
        }),
    };

    // Get the associated show to check ticket availability
    let show = _get_show(&payload.show_id);
    let mut updated_show = match show {
        Some(show) => show,
        None => return Err(Error::NotFound {
            msg: format!("a show with id={} not found", payload.show_id),
        }),
    };

    // Check if there are enough available tickets
    let available_tickets = updated_show.available_tickets;
    let diff_tickets = payload.num_tickets as i32 - booking.num_tickets as i32;
    if diff_tickets > available_tickets as i32 || payload.num_tickets == 0 {
        return Err(Error::NotEnoughTickets);
    }

    // Update available tickets after updating booking
    updated_show.available_tickets += diff_tickets as u32;
    do_insert_show(&updated_show);

    // Update booking information
    let mut updated_booking = booking;
    updated_booking.show_id = payload.show_id;
    updated_booking.user_id = payload.user_id;
    updated_booking.num_tickets = payload.num_tickets;

    // Update the booking in storage
    BOOKING_STORAGE.with(|service| service.borrow_mut().insert(id, updated_booking.clone()));
    Ok(updated_booking)
}

#[ic_cdk::update]
fn delete_booking(id: u64) -> Result<Booking, Error> {
    match BOOKING_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(booking) => {
            // Get the associated show to update available tickets
            let show = _get_show(&booking.show_id);
            if let Some(mut updated_show) = show {
                updated_show.available_tickets += booking.num_tickets;
                do_insert_show(&updated_show);
            }
            Ok(booking)
        }
        None => Err(Error::NotFound {
            msg: format!("a booking with id={} not found", id),
        }),
    }
}

// Helper method to perform booking insertion.
fn do_insert_booking(booking: &Booking) {
    BOOKING_STORAGE.with(|service| service.borrow_mut().insert(booking.id, booking.clone()));
}

// Additional helper method to get a booking by id. Used in get_booking.
fn _get_booking(id: &u64) -> Option<Booking> {
    BOOKING_STORAGE.with(|service| service.borrow().get(id))
}

// Common method to update show ticket availability
fn update_show_common(show: &mut Show, num_tickets_change: &u32) -> Result<(), Error> {
    // Check if there are enough available tickets
    if show.available_tickets < *num_tickets_change || *num_tickets_change == 0 {
        return Err(Error::NotEnoughTickets);
    }

    // Update available tickets after booking
    show.available_tickets -= *num_tickets_change;
    Ok(())
}

#[ic_cdk::query]
fn get_remaining_tickets(show_id: u64) -> Result<u32, Error> {
    let show = _get_show(&show_id);
    match show {
        Some(show) => Ok(show.available_tickets),
        None => Err(Error::NotFound {
            msg: format!("a show with id={} not found", show_id),
        }),
    }
}

// need this to generate candid
ic_cdk::export_candid!();
