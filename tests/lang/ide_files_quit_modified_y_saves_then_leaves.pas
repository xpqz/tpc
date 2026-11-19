program ide_files_quit_modified_y_saves_then_leaves;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('abc');
  EditLoop;
  Report('Q1.PAS');
end.
