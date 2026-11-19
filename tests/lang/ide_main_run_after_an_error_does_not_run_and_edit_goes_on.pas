program ide_main_run_after_an_error_does_not_run_and_edit_goes_on;
uses Crt, Buffer, View, Keys, Files, FHarness, Main;

begin
  Start('ERROR');
  RunIde;

end.
