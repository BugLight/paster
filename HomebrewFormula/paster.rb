class Paster < Formula
  version "0.1.1"
  desc "Easily paste from your terminal to services like Pastebin"
  homepage "https://github.com/BugLight/paster"
  license "MIT"

  on_macos do
    on_intel do
      url "https://github.com/BugLight/paster/releases/download/v#{version}/paster-v#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "05b3f584d6f51cbef7052a3b4c314a7007e8a8d63315f20f1a92cccdedbd6181"
    end

    on_arm do
      url "https://github.com/BugLight/paster/releases/download/v#{version}/paster-v#{version}-aarch64-apple-darwin.tar.gz"
      sha256 "d1a2ce9cb089c5582389a5067de52908acd31166558bd2068ecafb76354e9290"
    end
  end

  on_linux do
    url "https://github.com/BugLight/paster/releases/download/v#{version}/paster-v#{version}-x86_64-unknown-linux-musl.tar.gz"
    sha256 "8d4bc88891ecd9643cbd4c4d78edae2c8a25aa4bafdca8fd2b5447624a523223"
  end

  def install
    bin.install "paster"
  end
end
