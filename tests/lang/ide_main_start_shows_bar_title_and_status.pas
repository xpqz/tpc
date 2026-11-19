program ide_main_start_shows_bar_title_and_status;
uses Crt, Buffer, View, Keys, Files, FHarness, Main;

begin
  Start('program p;~begin~end.');
  RunIde;

end.
