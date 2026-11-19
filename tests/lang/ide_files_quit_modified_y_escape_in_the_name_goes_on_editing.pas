program ide_files_quit_modified_y_escape_in_the_name_goes_on_editing;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('abc');
  EditLoop;
  Report('');
end.
