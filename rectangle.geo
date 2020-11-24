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
