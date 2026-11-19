program swag_val_into_word_variables;
var
  w, code: Word;
begin
  Val('123', w, code);
  writeln(w, ' ', code);
  Val('1x', w, code);
  writeln(code);
end.
