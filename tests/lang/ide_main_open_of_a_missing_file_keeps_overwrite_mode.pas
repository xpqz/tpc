program ide_main_open_of_a_missing_file_keeps_overwrite_mode;
uses Crt, Buffer, View, Keys, Files, FHarness, Main;

begin
  Start('abc');
  RunIde;

end.
