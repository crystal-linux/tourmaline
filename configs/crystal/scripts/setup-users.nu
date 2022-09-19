def main [cfg] {
  $cfg | get users | each {|$it| create_user $it } | ignore
}

def create_user [user] {
  echo "This would create a user with:"
  echo $user
  echo
}