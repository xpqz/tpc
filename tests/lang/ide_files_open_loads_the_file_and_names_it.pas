program ide_files_open_loads_the_file_and_names_it;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('old text');
  EditLoop;
  Report('GOOD.TXT');
end.
