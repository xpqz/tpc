program TypedconstRecord;
type
  TPoint = record
    x, y: Integer;
  end;
  TPerson = record
    name: string[8];
    age: Byte;
    height: Real;
    initial: Char;
  end;
const
  origin: TPoint = (x: 0; y: 0);
  here: TPoint = (x: 3; y: -4);
  bob: TPerson = (name: 'Bob'; age: 41; height: 1.75; initial: 'B');
begin
  WriteLn(origin.x, ' ', origin.y);
  WriteLn(here.x, ' ', here.y);
  WriteLn(bob.name, ' ', bob.age, ' ', bob.height:0:2, ' ', bob.initial);
  here.x := here.x * 10;
  bob.age := bob.age + 1;
  bob.name := 'Robert Q';
  WriteLn(here.x, ' ', bob.age, ' ', bob.name);
end.
