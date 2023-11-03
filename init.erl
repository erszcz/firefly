-module(init).

-export([boot/1]).
-export([task/2]).

boot(_) ->
    erlang:display("hello!"),
    Self = self(),
    Pids = [ spawn(?MODULE, task, [Self, N]) || N <- [1,2,3,4,5] ],
    erlang:display("spawned all"),
    [ receive _ -> ok end || _ <- Pids ],
    erlang:display("joined all"),
    ok.

task(Self, N) ->
    erlang:display({task, Self, N}),
    Self ! ok.
