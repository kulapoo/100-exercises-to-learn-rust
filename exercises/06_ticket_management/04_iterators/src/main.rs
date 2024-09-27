use iterators::{Ticket, TicketStore};
use ticket_fields::test_helpers::{ticket_description, ticket_title};

fn main () {
    let mut store = TicketStore::new();
    store.add_ticket(Ticket {
        description: ticket_description(),
        title: ticket_title(),
        status: iterators::Status::Done
    });
    store.add_ticket(Ticket {
        description: ticket_description(),
        title: ticket_title(),
        status: iterators::Status::ToDo
    });
    for item in store {
        println!("{:#?}", item)
    }
}
