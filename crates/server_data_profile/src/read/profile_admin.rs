use server_data::define_cmd_wrapper_read;

mod profile_name_allowlist;
mod profile_text;
mod iterator;
mod report;

define_cmd_wrapper_read!(ReadCommandsProfileAdmin);

impl<'a> ReadCommandsProfileAdmin<'a> {
    pub fn profile_name_allowlist(
        self,
    ) -> profile_name_allowlist::ReadCommandsProfileNameAllowlist<'a> {
        profile_name_allowlist::ReadCommandsProfileNameAllowlist::new(self.0)
    }

    pub fn profile_text(self) -> profile_text::ReadCommandsProfileText<'a> {
        profile_text::ReadCommandsProfileText::new(self.0)
    }

    pub fn iterator(self) -> iterator::ReadCommandsProfileIterator<'a> {
        iterator::ReadCommandsProfileIterator::new(self.0)
    }

    pub fn report(self) -> report::ReadCommandsProfileReport<'a> {
        report::ReadCommandsProfileReport::new(self.0)
    }
}
