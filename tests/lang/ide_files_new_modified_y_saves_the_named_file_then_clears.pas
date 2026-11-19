program ide_files_new_modified_y_saves_the_named_file_then_clears;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('');
  EditLoop;
  Report('GOOD.TXT');
end.
