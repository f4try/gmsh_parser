// Gmsh project created on Tue Nov 24 23:05:45 2020
SetFactory("OpenCASCADE");
//+
Rectangle(1) = {0, 0, 0, 2, 1, 0};
//+
Physical Curve("ZERO") = {3, 1};
//+
Physical Curve("HIGH") = {4};
//+
Physical Curve("LOW") = {2};
//+
Transfinite Curve {3, 1} = 10 Using Progression 1;
//+
Transfinite Curve {4, 2} = 5 Using Progression 1;
//+
Physical Surface("Domain") = {1};
