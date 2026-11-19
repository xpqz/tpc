program ide_files_open_missing_after_a_good_open_keeps_that_file;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('');
  EditLoop;
  Report('GOOD.TXT');
end.
