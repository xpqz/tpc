program ide_main_ctrl_f9_error_does_not_run;
uses Crt, Buffer, View, Keys, Files, FHarness, Main;

begin
  Start('a~ERROR~c');
  RunIde;

end.
