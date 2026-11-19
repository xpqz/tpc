program ide_files_open_missing_file_shows_a_message_and_keeps_the_text;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('keep me~safe');
  EditLoop;
  Report('GOOD.TXT');
end.
