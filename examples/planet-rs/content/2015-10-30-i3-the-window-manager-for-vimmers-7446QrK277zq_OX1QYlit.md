+++
title = "i3: the window manager for vimmers"
date = "2015-10-30T00:00:00+00:00"

[extra]
author = "Vitiral"
link = "http://vitiral.github.com/linux/2015/10/30/30-i3-vim-mode.html"
+++
<p><a href="https://i3wm.org/">i3</a> is an ultra lightweight tiling window manager with <a href="http://i3wm.org/docs/">excellent documentation</a>. It follows the Arch Way in that it is configurable through a simple text based config file and is generally badass (well tested, simple to use, small codebase, etc). It is also fast and consumes almost no memory.</p>

<p>i3’s entire purpose is to be controlled through key commands. When you first install it, it gives you a well documented configuration file for you to edit. There are only a few changes we have to make to make i3 vim compatible.</p>

<p>One of the key benefits of using Arch and i3 is that there are no “random” keybindings – everything is documented in your i3 config. This makes it much easier to do things like define Alt+C/V to be copy/paste (for example) – we will get to that when we discuss urxvt</p>

<h1 id="open-i3-or-configi3config">Open <code class="language-plaintext highlighter-rouge">~/.i3</code> or <code class="language-plaintext highlighter-rouge">~/.config/i3/config</code></h1>
<div class="language-plaintext highlighter-rouge"><div class="highlight"><pre class="highlight"><code># Mod=Alt -- I use alt so it doesn't conflict with vim or tmux but is easy to reach
set $mod Mod1

# kill focused window (vim delete, follows vimperator settings more than vim)
bindsym $mod+d kill

# dmenu is how you start programs -- make it the same as vim command mode
bindsym $mod+semicolon exec dmenu_run

# i3 uses jkl; instead of hjkl -- fix that
bindsym $mod+h focus left
bindsym $mod+j focus down
bindsym $mod+k focus up
bindsym $mod+l focus right

# move focused window
bindsym $mod+Shift+h move left
bindsym $mod+Shift+j move down
bindsym $mod+Shift+k move up
bindsym $mod+Shift+l move right

# These are more like the tmux config we will get to later, but are intuitive
bindsym $mod+backslash  split horizontal
bindsym $mod+bar        split horizontal
bindsym $mod+apostrophe split vertical
bindsym $mod+quotedbl   split vertical
bindsym $mod+minus      split vertical
</code></pre></div></div>

<p>You can also see more settings (and up to date settings) I have at my <a href="https://github.com/vitiral/dotfiles/blob/master/config/i3/config">i3 config</a></p>