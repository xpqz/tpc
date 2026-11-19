program ide_main_quit_discards_after_no;
uses Crt, Buffer, View, Keys, Files, FHarness, Main;

begin
  Start('abc');
  RunIde;
  Report('OUT.PAS');
end.
