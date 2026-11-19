program ide_main_ctrl_f9_good_shows_the_exit_code;
uses Crt, Buffer, View, Keys, Files, FHarness, Main;

begin
  Start('a~b~c');
  RunIde;

end.
