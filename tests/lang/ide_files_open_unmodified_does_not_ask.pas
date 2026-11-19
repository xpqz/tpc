program ide_files_open_unmodified_does_not_ask;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('same');
  EditLoop;
  Report('GOOD.TXT');
end.
