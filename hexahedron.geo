//+
SetFactory("OpenCASCADE");
Box(1) = {0, 0, 0, 1, 1, 10};
//+//+
Transfinite Curve {6, 12, 2, 2, 10, 11, 4, 9, 8} = 2 Using Progression 1;
//+
Transfinite Curve {3, 1, 9, 7, 5} = 2 Using Progression 1;
//+
Physical Surface("Inlet") = {6};
//+
Physical Surface("Outlet") = {5};
//+
Physical Surface("Wall") = {2, 4, 1, 3};
//+
Physical Volume("Channel") = {1};
