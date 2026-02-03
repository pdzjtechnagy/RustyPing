class Rustyping < Formula
  desc "Advanced TUI Ping Tool"
  homepage "https://github.com/pdzjtechnagy/RustyPing"
  url "https://github.com/pdzjtechnagy/RustyPing/releases/download/v2.4.4/rustyping-macos-v2.4.4.tar.gz"
  sha256 "REPLACE_WITH_ACTUAL_SHA256"
  version "2.4.4"
  license "MIT"

  def install
    bin.install "rustyping"
  end

  test do
    system "#{bin}/rustyping", "--version"
  end
end
