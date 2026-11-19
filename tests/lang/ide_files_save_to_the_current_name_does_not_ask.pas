program ide_files_save_to_the_current_name_does_not_ask;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('');
  EditLoop;
  Report('GOOD.TXT');
end.
