program ide_files_save_replaces_an_existing_file;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('x');
  EditLoop;
  Report('GOOD.TXT');
end.
