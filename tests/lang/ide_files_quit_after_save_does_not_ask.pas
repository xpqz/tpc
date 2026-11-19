program ide_files_quit_after_save_does_not_ask;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('abc');
  EditLoop;
  Report('Q2.PAS');
end.
