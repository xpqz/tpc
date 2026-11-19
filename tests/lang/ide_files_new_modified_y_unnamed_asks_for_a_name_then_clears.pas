program ide_files_new_modified_y_unnamed_asks_for_a_name_then_clears;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('abc');
  EditLoop;
  Report('T1.PAS');
end.
