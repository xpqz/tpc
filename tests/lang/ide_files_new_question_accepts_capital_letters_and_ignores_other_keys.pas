program ide_files_new_question_accepts_capital_letters_and_ignores_other_keys;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('abc');
  EditLoop;
  Report('T2.PAS');
end.
