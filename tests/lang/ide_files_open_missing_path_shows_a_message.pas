program ide_files_open_missing_path_shows_a_message;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('keep me');
  EditLoop;
  Report('GOOD.TXT');
end.
