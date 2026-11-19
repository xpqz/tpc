program ide_files_open_missing_name_while_modified_still_asks_before_loading;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('mine');
  EditLoop;
  Report('GOOD.TXT');
end.
