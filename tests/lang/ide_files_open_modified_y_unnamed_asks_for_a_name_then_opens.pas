program ide_files_open_modified_y_unnamed_asks_for_a_name_then_opens;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('draft');
  EditLoop;
  Report('GOOD.TXT SAVED.PAS');
end.
