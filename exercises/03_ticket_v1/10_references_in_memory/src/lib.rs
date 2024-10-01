pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    #[test]
    fn u16_ref_size() {
        assert_eq!(size_of::<&u16>(), size_of::<usize>());
    }

    #[test]
    fn u64_mut_ref_size() {
        assert_eq!(size_of::<&mut u64>(), size_of::<usize>());
    }

    #[test]
    fn ticket_ref_size() {
        assert_eq!(size_of::<&Ticket>(), size_of::<usize>());
    }
}
