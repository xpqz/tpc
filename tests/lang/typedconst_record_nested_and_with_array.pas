program TypedconstRecordNestedAndWithArray;
type
  TPoint = record
    x, y: Integer;
  end;
  TLine = record
    a, b: TPoint;
    tag: string[4];
  end;
  TBox = record
    corners: array[1..3] of TPoint;
    id: Integer;
  end;
const
  line: TLine = (a: (x: 1; y: 2); b: (x: 3; y: 4); tag: 'ab');
  box: TBox = (corners: ((x: 1; y: 1), (x: 2; y: 2), (x: 3; y: 3)); id: 7);
begin
  WriteLn(line.a.x, line.a.y, line.b.x, line.b.y, ' ', line.tag);
  WriteLn(box.corners[2].x, ' ', box.corners[3].y, ' ', box.id);
  line.b.y := 40;
  box.corners[1].x := 10;
  WriteLn(line.b.y, ' ', box.corners[1].x);
end.
