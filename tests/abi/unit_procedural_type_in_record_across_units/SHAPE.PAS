unit Shape;
{$F+}
interface
type
  TAreaFn = function(w, h: Integer): Integer;
  TShape = record
    name: string[10];
    w, h: Integer;
    area: TAreaFn;
  end;
implementation
end.
