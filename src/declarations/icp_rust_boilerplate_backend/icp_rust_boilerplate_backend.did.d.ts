import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Booking {
  'id' : bigint,
  'num_tickets' : number,
  'user_id' : bigint,
  'show_id' : bigint,
}
export interface BookingPayload {
  'num_tickets' : number,
  'user_id' : bigint,
  'show_id' : bigint,
}
export type Error = { 'InvalidInput' : null } |
  { 'NotEnoughTickets' : null } |
  { 'NotFound' : { 'msg' : string } };
export type Result = { 'Ok' : Booking } |
  { 'Err' : Error };
export type Result_1 = { 'Ok' : Show } |
  { 'Err' : Error };
export type Result_2 = { 'Ok' : number } |
  { 'Err' : Error };
export interface Show {
  'id' : bigint,
  'title' : string,
  'available_tickets' : number,
  'end_time' : bigint,
  'start_time' : bigint,
  'genre' : string,
  'total_tickets' : number,
}
export interface ShowPayload {
  'title' : string,
  'end_time' : bigint,
  'start_time' : bigint,
  'genre' : string,
  'total_tickets' : number,
}
export interface _SERVICE {
  'add_booking' : ActorMethod<[BookingPayload], [] | [Booking]>,
  'add_show' : ActorMethod<[ShowPayload], [] | [Show]>,
  'delete_booking' : ActorMethod<[bigint], Result>,
  'delete_show' : ActorMethod<[bigint], Result_1>,
  'get_booking' : ActorMethod<[bigint], Result>,
  'get_remaining_tickets' : ActorMethod<[bigint], Result_2>,
  'get_show' : ActorMethod<[bigint], Result_1>,
  'update_booking' : ActorMethod<[bigint, BookingPayload], Result>,
  'update_show' : ActorMethod<[bigint, ShowPayload], Result_1>,
}
