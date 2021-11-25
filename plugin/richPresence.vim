if !exists("s:presenceJobId")
    let s:presenceJobId = 0
endif

let richPresence#execdir = resolve(expand('<sfile>:p:h') . '/..')

if exists("g:rich_presence_binary")
    let s:bin = richPresence#execdir . "/" . substitute(g:rich_presence_binary, "^\/", "", "")
else
    echoerr "Please provide name of the rich presence binary"
    finish
endif

function! s:connect()
    let id = richPresence#init()

    call s:loadDefaultConfig()

    if 0 == id
        echoerr "Failed to start Rich Presence"
    elseif -1 == id
        echoerr "Rich Presence binary cannot be executed"
    else
        let s:presenceJobId = id

        command! UpdateRichPresence silent! call rpcnotify(s:presenceJobId, 'update')
        command! ReconnectRichPresence call s:RestartWrapper(function('rpcnotify'), s:presenceJobId, 'reconnect')
        command! DisconnectRichPresence call s:RestartWrapper(function('rpcnotify'), s:presenceJobId, 'disconnect')
        command! RestartRichPresence call s:restartPlugin()
    endif
endfunction

function! s:loadDefaultConfig()
    if !exists("g:nvimsence_details")
        let g:nvimsence_details = "{project}/{filename}"
    endif

    if !exists("g:nvimsence_state")
        let g:nvimsence_state = "{filesize} [{lines} LOC]"
    endif

    if !exists("g:nvimsence_show_elapsed")
        let g:nvimsence_show_elapsed = 1
    endif

    if !exists("g:nvimsence_show_buttons")
        let g:nvimsence_show_buttons = 1
    endif
endfunction

function! s:RestartWrapper(callback, ...)
    try
        silent! call a:callback(a:1, a:2)
    catch
        silent! call s:restartPlugin()
        call a:callback(a:1, a:2)
    endtry
endfunction

function! s:restartPlugin()
    call jobstop(s:presenceJobId)
    let s:presenceJobId = 0
    let s:presenceJobId = richPresence#init()
endfunction

function! richPresence#init()
    if s:presenceJobId == 0
        let jobId = jobstart([s:bin], { 'rpc': v:true })
        return jobId
    else
        return s:presenceJobId
    endif
endfunction

call s:connect()

augroup Presence
    autocmd!
    autocmd BufNewFile,BufRead,BufEnter * :UpdateRichPresence
augroup END
