import os
import subprocess

CWD = os.getcwd()

def nfo_dir_check():
    if os.path.exists('/usr/share/rusicsetup/rusicsetup/nfo'):
        return True
    else:
        return False

def rm_nfo_dir():
    if os.path.exists('/usr/share/rusicsetup/rusicsetup/nfo'):
        subprocess.call(['rm', '-r', '/usr/share/rusicsetup/rusicsetup/nfo'])
        print('NFO files removed')

def db_dir_check():
    if os.path.exists('/usr/share/rusicsetup/rusicsetup/db'):
        return True
    else:
        return False

def rm_db_dir():
    if os.path.exists('/usr/share/rusicsetup/rusicsetup/db'):
        subprocess.call(['rm', '-rf', '/usr/share/rusicsetup/rusicsetup/db'])
        print('DB files removed')

def thumb_dir_check():
    if os.path.exists('/usr/share/rusicsetup/rusicsetup/thumbs'):
        return True
    else:
        return False

def rm_thumb_dir():
    if os.path.exists('/usr/share/rusicsetup/rusicsetup/thumbs'):
        subprocess.call(['rm', '-rf', '/usr/share/rusicsetup/rusicsetup/thumbs'])
        print('Thumbs files removed')

def install_dir_check():
    if os.path.exists('/usr/share/rusicsetup/rusicsetup'):
        return True
    else:
        return False

def git_pull():
    os.chdir('/usr/share/rusicsetup/rusicsetup')
    subprocess.call(['git', 'pull'])
    os.chdir(CWD)

def git_clone():
    os.chdir('/usr/share/rusicsetup')
    subprocess.call(['git', 'clone', 'https://github.com/cjsmocjsmo/rusicsetup.git'])
    os.chdir(CWD)

def run_cargo():
    os.chdir('/usr/share/rusicsetup/rusicsetup')
    subprocess.call(['cargo', 'run', '--release'])
    os.chdir(CWD)

if __name__ == '__main__':
    if install_dir_check():
        rm_db_dir()
        rm_nfo_dir()
        rm_thumb_dir()
        git_pull()
    else:
        git_clone()

    run_cargo()