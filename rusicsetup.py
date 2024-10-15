import os
import subprocess

CWD = os.getcwd()

def rm_nfo_dir():
    if os.path.exists('/usr/share/rusicsetup/rusicsetup/nfo'):
        subprocess.call(['rm', '-r', '/usr/share/rusicsetup/rusicsetup/nfo'])
        print('NFO files removed')

def create_nfo_dir():
    if not os.path.exists('/usr/share/rusicsetup/rusicsetup/nfo'):
        os.makedirs('/usr/share/rusicsetup/rusicsetup/nfo')
        print('NFO files created')

def rm_db_dir():
    if os.path.exists('/usr/share/rusicsetup/rusicsetup/db'):
        subprocess.call(['rm', '-rf', '/usr/share/rusicsetup/rusicsetup/db'])
        subprocess.call(['mkdir', '/usr/share/rusicsetup/rusicsetup/db'])
        subprocess.call(['touch', '/usr/share/rusicsetup/rusicsetup/db/rusic.db'])
        print('DB files removed')

def create_db_dir():
    if not os.path.exists('/usr/share/rusicsetup/rusicsetup/db'):
        os.makedirs('/usr/share/rusicsetup/rusicsetup/db')
        print('DB files created')

def rm_thumb_dir():
    if os.path.exists('/usr/share/rusicsetup/rusicsetup/thumbs'):
        subprocess.call(['rm', '-rf', '/usr/share/rusicsetup/rusicsetup/thumbs'])
        print('Thumbs files removed')

def create_thumb_dir():
    if not os.path.exists('/usr/share/rusicsetup/rusicsetup/thumbs'):
        os.makedirs('/usr/share/rusicsetup/rusicsetup/thumbs')
        print('Thumbs files created')

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
        create_db_dir()
        create_nfo_dir()
        create_thumb_dir()
        run_cargo()
    else:
        git_clone()
        run_cargo()