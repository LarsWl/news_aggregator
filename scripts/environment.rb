FILENAME = "/home/egor/rust/news_aggregator/.env.local"

File.read(FILENAME).each_line do |line|
  next if line.start_with?("#")

  env_name = line.split("=").first
  env_value = line.split("=").last

  env_value.chop! if env_value[-1] == "\n"

  ENV[env_name] = env_value
end
