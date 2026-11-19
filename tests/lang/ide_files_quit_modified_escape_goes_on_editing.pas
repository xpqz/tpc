program ide_files_quit_modified_escape_goes_on_editing;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('abc');
  EditLoop;
  Report('');
end.
