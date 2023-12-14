export const idlFactory = ({ IDL }) => {
  const BookingPayload = IDL.Record({
    'num_tickets' : IDL.Nat32,
    'user_id' : IDL.Nat64,
    'show_id' : IDL.Nat64,
  });
  const Booking = IDL.Record({
    'id' : IDL.Nat64,
    'num_tickets' : IDL.Nat32,
    'user_id' : IDL.Nat64,
    'show_id' : IDL.Nat64,
  });
  const ShowPayload = IDL.Record({
    'title' : IDL.Text,
    'end_time' : IDL.Nat64,
    'start_time' : IDL.Nat64,
    'genre' : IDL.Text,
    'total_tickets' : IDL.Nat32,
  });
  const Show = IDL.Record({
    'id' : IDL.Nat64,
    'title' : IDL.Text,
    'available_tickets' : IDL.Nat32,
    'end_time' : IDL.Nat64,
    'start_time' : IDL.Nat64,
    'genre' : IDL.Text,
    'total_tickets' : IDL.Nat32,
  });
  const Error = IDL.Variant({
    'InvalidInput' : IDL.Null,
    'NotEnoughTickets' : IDL.Null,
    'NotFound' : IDL.Record({ 'msg' : IDL.Text }),
  });
  const Result = IDL.Variant({ 'Ok' : Booking, 'Err' : Error });
  const Result_1 = IDL.Variant({ 'Ok' : Show, 'Err' : Error });
  const Result_2 = IDL.Variant({ 'Ok' : IDL.Nat32, 'Err' : Error });
  return IDL.Service({
    'add_booking' : IDL.Func([BookingPayload], [IDL.Opt(Booking)], []),
    'add_show' : IDL.Func([ShowPayload], [IDL.Opt(Show)], []),
    'delete_booking' : IDL.Func([IDL.Nat64], [Result], []),
    'delete_show' : IDL.Func([IDL.Nat64], [Result_1], []),
    'get_booking' : IDL.Func([IDL.Nat64], [Result], ['query']),
    'get_remaining_tickets' : IDL.Func([IDL.Nat64], [Result_2], ['query']),
    'get_show' : IDL.Func([IDL.Nat64], [Result_1], ['query']),
    'update_booking' : IDL.Func([IDL.Nat64, BookingPayload], [Result], []),
    'update_show' : IDL.Func([IDL.Nat64, ShowPayload], [Result_1], []),
  });
};
export const init = ({ IDL }) => { return []; };
