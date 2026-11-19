program ide_files_open_escape_cancels;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('stay');
  EditLoop;
  Report('GOOD.TXT');
end.
