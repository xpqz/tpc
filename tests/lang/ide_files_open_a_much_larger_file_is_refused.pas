program ide_files_open_a_much_larger_file_is_refused;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('old text');
  EditLoop;
  Report('');
end.
