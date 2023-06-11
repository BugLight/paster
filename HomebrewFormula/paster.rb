class Paster < Formula
  version "0.1.1"
  desc "Easily paste from your terminal to services like Pastebin"
  homepage "https://github.com/BugLight/paster"
  license "MIT"

  on_macos do
    on_intel do
      url "https://github.com/BugLight/paster/releases/download/v#{version}/paster-v#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "ad21ced0357c26b3d1410698785608f1c0a463e067cb3cc36264c913963b500a"
    end

    on_arm do
      url "https://github.com/BugLight/paster/releases/download/v#{version}/paster-v#{version}-aarch64-apple-darwin.tar.gz"
      sha256 "4e01175c5a4b4f1c7cae94933d9c960de0d7b5274ff902adb1ded734a8104cee"
    end
  end

  on_linux do
    url "https://github.com/BugLight/paster/releases/download/v#{version}/paster-v#{version}-x86_64-unknown-linux-musl.tar.gz"
    sha256 "78561ae0760ae4f1f4eecbdfc0a9b2189a6eb565053872fa2e0ede21da025290"
  end

  def install
    bin.install "paster"
  end
end
