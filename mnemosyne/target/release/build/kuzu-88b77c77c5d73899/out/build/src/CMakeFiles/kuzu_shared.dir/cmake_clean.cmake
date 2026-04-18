file(REMOVE_RECURSE
  "libkuzu.dylib"
  "libkuzu.pdb"
)

# Per-language clean rules from dependency scanning.
foreach(lang CXX)
  include(CMakeFiles/kuzu_shared.dir/cmake_clean_${lang}.cmake OPTIONAL)
endforeach()
