def main [cfg] {
  echo "Running after creating users"
  $TRM_CONFIG | describe
  $TRM_CONFIG
}