program ide_main_title_survives_a_cancelled_open;
uses Crt, Buffer, View, Keys, Files, FHarness, Main;

begin
  Start('abc');
  RunIde;

end.
