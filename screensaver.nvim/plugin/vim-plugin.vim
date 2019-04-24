if !exists('s:jobid')
  let s:jobid = 0
endif

let s:scriptdir = resolve(expand('<sfile>:p:h') . '/..')
let s:bin = s:scriptdir . '/target/release/vim-plugin'

function! s:connect()
  let s:jobid = s:job_start()
  if 0 == s:jobid
    echoerr "vim-plugin cannot start rpc process"
  elseif -1 == s:jobid
    echoerr "vim-plugin rpc process is not executable"
  else
    call s:au()
  endif
endfunction

function! s:job_start()
  return jobstart([s:bin], {
        \'rpc': v:true,
        \'on_stderr': function('s:on_stderr')
        \})
endfunction

function! s:on_stderr(id, data, event) dict
  echom 'stderr: ' . join(a:data, "\n")
  call s:job_stop()
endfunction

function! s:job_stop()
  call s:rpcnotify('leavin')
  let s:jobid = 0
endfunction

function! s:au()
  augroup vim-plugin
    autocmd!
    autocmd FocusGained * :call s:rpcnotify('stop')
    autocmd FocusLost * :call s:rpcnotify('start')
    autocmd VimLeavePre * :call s:job_stop()
  augroup END
endfunction

function! s:rpcnotify(k)
  if s:jobid > 0
    call rpcnotify(s:jobid, a:k)
  end
endfunction

function! s:job_restart()
    call s:job_stop()
    call s:connect()
    echomsg "restarted dat binary for yall"
endfunction

command! Screensaver :call s:connect()
command! ScreensaverOff :call s:job_stop()
