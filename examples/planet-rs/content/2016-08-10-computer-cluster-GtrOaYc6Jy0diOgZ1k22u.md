+++
title = "Computer Cluster"
date = "2016-08-10T00:00:00+00:00"

[extra]
author = "Rust — Jim Turner"
link = "https://jim.turner.link/pages/computer-cluster/"
+++
<p>In my lab at <a href="https://duke.edu/">Duke University</a>, we had a lot of old
computers from prior research projects that were no longer being used. I
volunteered to put them together into a cluster for the lab to use for
computationally-intensive tasks. I didn&rsquo;t know anything about cluster computing
before this project, so it was a great experience learning how to put together
and use a computer cluster.</p>

<p>If you&rsquo;re new to cluster computing and are interested in setting up your own
small computer cluster, the following overview may be helpful.</p>

<h2 id="hardware-network">Hardware &amp; Network</h2>

<p>The cluster has seven x86-64 desktop computers of varying age with a range of
processors and memory capacities. They are all connected with a
single
<a href="https://www.amazon.com/gp/product/B001EVGIYG/">8-port unmanaged network switch</a> that
is connected to Duke&rsquo;s network. This is a photograph of the cluster:</p>

<figure>
    <img src="https://jim.turner.link/media/computer_cluster_photo___default.jpg"
srcset="




/media/computer_cluster_photo___1000w.jpg 1000w,
/media/computer_cluster_photo___150w.jpg 150w,
/media/computer_cluster_photo___219w.jpg 219w,
/media/computer_cluster_photo___320w.jpg 320w,
/media/computer_cluster_photo___468w.jpg 468w,
/media/computer_cluster_photo___684w.jpg 684w"

 


sizes="(min-width: 32em) 32em, 100vw"
style="width:32em"

    alt="Photograph of seven desktop computers of different types on the floor of the lab, connected with a single network switch.">
    <figcaption>
        Photograph of the computer cluster. Image © 2016 Jim Turner and licensed under
        <a title="Creative Commons Attribution-ShareAlike 4.0 International License"
        href="https://creativecommons.org/licenses/by-sa/4.0/">CC BY‑SA 4.0</a>.
    </figcaption>
</figure>

<p>Six of the computers (<code>dsg01</code>, <code>dsg03</code>, <code>dsg04</code>, …, <code>dsg07</code>) are
compute nodes, and the remaining one (<code>dsg02</code>) is the login node, SLURM
controller, and file server. This is the network topology:</p>

<figure>
    <img src="https://jim.turner.link/media/computer_cluster_network.svg" style="width:37em"
    alt="The nodes are connected to a single switch. That switch is connected to Duke's network, which is separated from the Internet by a firewall. Users can connect to Duke's network directly or, if they are elsewhere on the Internet, through Duke's VPN.">
    <figcaption>
        Network topology of the cluster and users. Image © 2016 Jim Turner and licensed under
        <a title="Creative Commons Attribution-ShareAlike 4.0 International License"
        href="https://creativecommons.org/licenses/by-sa/4.0/">CC BY‑SA 4.0</a>.
    </figcaption>
</figure>

<h2 id="software">Software</h2>

<p>The hardest part of setting up the cluster was figuring out what software to
use and how to configure it. Since I was unfamiliar with cluster computing, I
strongly favored projects with good documentation that were fairly easy to set
up. I decided on the following:</p>

<ul>
<li><p><a href="https://www.debian.org/">Debian stable</a> for the OS. It&rsquo;s free software, is
reliable, and has long-term support. The Debian project also works very hard
to minimize changes to Debian stable, which reduces the work required to
administer the cluster.</p></li>

<li><p><a href="https://www.gluster.org/">Gluster</a> for the shared file system (for users&rsquo;
home directories).
The <a href="https://gluster.readthedocs.io/en/latest/">Gluster documentation</a> is
pretty good, so I found it easier to set up than the alternatives. It&rsquo;s also
a distributed file system, so if I need to add more storage capacity or speed
up transfer rates in the future, I can add more storage nodes.</p></li>

<li><p><a href="https://slurm.schedmd.com/">SLURM</a> for scheduling and resource management.
It is straightforward to set up<sup class="footnote-ref" id="fnref:1"><a rel="footnote" href="#fn:1">1</a></sup>, provides all the functionality I need,
and is popular.</p></li>

<li><p><a href="https://www.mpich.org/">MPICH</a> as the MPI implementation. Supposedly, it
<a href="https://wiki.mpich.org/mpich/index.php/Using_the_Hydra_Process_Manager#Resource_Managers_and_Launchers">automatically detects</a>
and integrates with SLURM, but I haven&rsquo;t tested this myself.</p></li>

<li><p><a href="https://dun.github.io/munge/">MUNGE</a> for hosts to authenticate each other
(needed for SLURM). This is easy to set up.</p></li>

<li><p><a href="http://ganglia.info/">Ganglia</a> for historical performance monitoring. The
documentation wiki appears to no longer exist, but the <a href="https://github.com/ganglia/monitor-core/wiki/Ganglia-Quick-Start">Ganglia quick start</a> was
sufficient to set it up.</p></li>

<li><p><a href="http://www.sphinx-doc.org/en/stable/">Sphinx</a> to build the documentation
for the cluster (hosted on the head node).</p></li>

<li><p><a href="https://httpd.apache.org/">Apache</a> as the web server on the head node to
host the documentation and Ganglia. Debian makes setting up Apache very easy.</p></li>

<li><p><a href="http://www.openssh.com/">OpenSSH</a> for users to connect to the cluster and
transfer files with SFTP. I also set up passwordless (key-based)
authentication for all users between hosts for MPICH.</p></li>
</ul>

<p>I installed additional software for users to develop and run their programs,
including:</p>

<ul>
<li><a href="https://docs.continuum.io/anaconda/">Miniconda</a> for the Python environment
because it&rsquo;s the easiest way to get up-to-date Python packages on Debian
stable.</li>
<li><a href="https://gcc.gnu.org/">GNU Compiler Collection (GCC)</a> for the C/C++/Fortran
environment.</li>
<li><a href="https://www.gnu.org/software/octave/">GNU Octave</a> as a free alternative to
MATLAB.</li>
<li><a href="https://www.mathworks.com/products/matlab.html">MATLAB</a>, because the other
researchers in my lab use it.</li>
</ul>

<h2 id="usage">Usage</h2>

<p>If you&rsquo;re unfamiliar with computer clusters, it&rsquo;s helpful to know how they work
from the user&rsquo;s perspective. This is how the small cluster I built is set up:</p>

<p>The user has access to his/her home directory and the <code>/tmp</code> directory on each
node. The user&rsquo;s home directory is shared across the nodes with Gluster, so all
programs and input/output files in the user&rsquo;s home directory are available on
all nodes. To run a job on the cluster:</p>

<ol>
<li><p>The user transfers his/her program and input data to the login node with
SFTP.</p></li>

<li><p>The user SSHes into the cluster&rsquo;s login node. He/she can run inexpensive
tasks on the login node, such as compiling small programs. However, for
computationally-intensive tasks, the user should submit a job with SLURM to
run on the compute nodes.</p></li>

<li><p>On the login node, the user can use the following SLURM commands:</p>

<ul>
<li><code>srun</code> to run a single job and wait for it to complete,</li>
<li><code>salloc</code> to allocate resources (primarily for an interactive job), or</li>
<li><code>sbatch</code> to schedule a batch job for execution.</li>
</ul></li>

<li><p>When the necessary resources (i.e. processors and memory) become available
on the compute nodes, SLURM starts the job on the available compute nodes.</p></li>

<li><p>The user can cancel the job with <code>scancel</code> or check its status with
<code>squeue</code>.</p></li>

<li><p>If the user submitted a batch job, SLURM saves the standard output and
standard error from the job to the specified location (typically the user
would specify files in his/her home directory). The program being run can
also save output files itself to the user&rsquo;s home directory, because the
user&rsquo;s home directory is transparently synchronized between the nodes with
Gluster.</p></li>

<li><p>When the job is complete, the user can download the output files from the
login node with SFTP.</p></li>
</ol>

<h2 id="configuration-management-testing">Configuration Management &amp; Testing</h2>

<p>One of my goals was to automate the installation and configuration of the
cluster as much as possible in order to simplify maintenance and enable version
control of the configuration. For installation and configuration, I&rsquo;m using:</p>

<ul>
<li><p><a href="https://docs.ansible.com/ansible/">Ansible</a> for configuration management.
Ansible is relatively simple to set up, is extensible, and works well enough
for my needs.</p></li>

<li><p><a href="https://git-scm.com/">Git</a> for version control of the configuration.</p></li>

<li><p><a href="https://www.debian.org/releases/stable/amd64/apb.en.html">Debian preseeding</a> for the initial
installation of the OS. Unfortunately, preseeding is not well documented, but
I was successful basing the template off of <a href="https://www.debian.org/releases/jessie/example-preseed.txt">this example</a> and the
<a href="https://salsa.debian.org/installer-team/debian-installer/blob/master/doc/devel/partman-auto-recipe.txt">partman-auto documentation</a>.</p></li>

<li><p><a href="http://jinja.pocoo.org/docs/dev/intro/">Jinja</a> for generating the preseed
files by filling a template with variables parsed from Ansible.</p></li>

<li><p><a href="https://www.gnu.org/software/make/">GNU Make</a> for automating the build
process of the configuration and test images.</p></li>
</ul>

<p>Since users could be running jobs on the cluster, I needed a way to test
changes that didn&rsquo;t interfere with the actual cluster. I&rsquo;m using the following
additional software to test the configuration with a network of virtual
machines on my laptop:</p>

<ul>
<li><a href="https://www.packer.io/">Packer</a> to build clean Debian virtual machine images
with the preseed files.</li>
<li><a href="https://www.vagrantup.com/">Vagrant</a> to start and provision the virtual
machines with Ansible.</li>
<li><a href="https://www.virtualbox.org/">VirtualBox</a> to run the virtual machines.</li>
</ul>

<h2 id="documentation-sustainability">Documentation &amp; Sustainability</h2>

<p>One of my goals when building the cluster was to make it sustainable after I
leave Duke. As a result, I automated as much of the configuration as possible
and documented everything. I&rsquo;m using <a href="http://www.sphinx-doc.org/en/stable/">Sphinx</a> for documentation, and I&rsquo;m keeping the
configuration and documentation on Duke&rsquo;s GitLab instance.</p>

<h2 id="other-resources">Other Resources</h2>

<p>If you&rsquo;d like to set up your own small cluster, the following resources may be
helpful:</p>

<ul>
<li><p>The documentation for the software I listed above.</p></li>

<li><p>Partway through the project, I found <a href="https://github.com/ajdecon/ansible-simple-slurm-cluster">ajdecon&rsquo;s ansible-simple-slurm-cluster
repository</a> with
Ansible roles to set up a SLURM-based cluster. I made some different
decisions than ajdecon, but his example was really helpful as an outline of
what to do.</p></li>

<li><p>Many universities have documentation about their SLURM clusters; this is
helpful to learn how users interact with the cluster. For example, <a href="https://portal.tacc.utexas.edu/user-guides/stampede">UT Austin
has good documentation for their Stampede cluster</a>.</p></li>
</ul>
<div class="footnotes">

<hr />

<ol>
<li id="fn:1">To generate an initial configuration, use one of the configuration builders, which are available at <code>/usr/share/doc/slurmctld/slurm-wlm-configurator.easy.html</code> and <code>/usr/share/doc/slurmctld/slurm-wlm-configurator.html</code> once you have <code>slurmctld</code> installed. Look at the man page for <code>slurm.conf(5)</code> for more information about the options.
 <a class="footnote-return" href="#fnref:1"><sup>[return]</sup></a></li>
</ol>
</div>