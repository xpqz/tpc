program ide_files_open_empty_name_cancels;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('stay');
  EditLoop;
  Report('GOOD.TXT');
end.
