program ide_files_open_an_empty_file;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('was here');
  EditLoop;
  Report('EMPTY.TXT');
end.
