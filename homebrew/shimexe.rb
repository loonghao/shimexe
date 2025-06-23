class Shimexe < Formula
  desc "The Modern Executable Shim Manager"
  homepage "https://github.com/loonghao/shimexe"
  version "0.3.5"
  license "MIT"

  if OS.mac?
    if Hardware::CPU.arm?
      url "https://github.com/loonghao/shimexe/releases/download/v#{version}/shimexe-#{version}-aarch64-apple-darwin.tar.xz"
      sha256 "PLACEHOLDER_ARM64_SHA256"
    else
      url "https://github.com/loonghao/shimexe/releases/download/v#{version}/shimexe-#{version}-x86_64-apple-darwin.tar.xz"
      sha256 "PLACEHOLDER_X86_64_SHA256"
    end
  elsif OS.linux?
    if Hardware::CPU.arm?
      url "https://github.com/loonghao/shimexe/releases/download/v#{version}/shimexe-#{version}-aarch64-unknown-linux-gnu.tar.xz"
      sha256 "PLACEHOLDER_LINUX_ARM64_SHA256"
    else
      url "https://github.com/loonghao/shimexe/releases/download/v#{version}/shimexe-#{version}-x86_64-unknown-linux-gnu.tar.xz"
      sha256 "PLACEHOLDER_LINUX_X86_64_SHA256"
    end
  end

  def install
    bin.install "shimexe"
    
    # Install shell completions if available
    if (buildpath/"completions").exist?
      bash_completion.install "completions/shimexe.bash" => "shimexe"
      zsh_completion.install "completions/_shimexe"
      fish_completion.install "completions/shimexe.fish"
    end
    
    # Install man page if available
    if (buildpath/"man").exist?
      man1.install "man/shimexe.1"
    end
  end

  test do
    system "#{bin}/shimexe", "--version"
    system "#{bin}/shimexe", "--help"
    
    # Test basic functionality
    system "#{bin}/shimexe", "init"
    assert_predicate testpath/".shimexe", :exist?
  end
end
