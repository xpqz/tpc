program ide_files_open_modified_y_unnamed_escape_in_the_name_cancels_the_open;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('draft');
  EditLoop;
  Report('GOOD.TXT SAVED.PAS');
end.
