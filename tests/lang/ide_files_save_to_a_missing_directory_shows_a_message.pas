program ide_files_save_to_a_missing_directory_shows_a_message;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('');
  EditLoop;
  Report('NODIR\X.PAS');
end.
