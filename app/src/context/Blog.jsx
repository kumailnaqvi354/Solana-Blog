import { createContext, useContext, useState, useEffect, useMemo } from "react";
import * as anchor from "@project-serum/anchor";
import {
  useAnchorWallet,
  useConnection,
  useWallet,
} from "@solana/wallet-adapter-react";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { getAvatarUrl } from "src/functions/getAvatarUrl";
import { getRandomName } from "src/functions/getRandomName";
import idl from "../idl.json";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { utf8 } from "@project-serum/anchor/dist/cjs/utils/bytes";
import { publicKey } from "@project-serum/anchor/dist/cjs/utils";
import { async } from "rxjs";

const BlogContext = createContext();

// GET Program Key
const PROGRAM_KEY = new PublicKey(idl.metadata.address);

export const useBlog = () => {
  const context = useContext(BlogContext);
  // const

  if (!context) {
    throw new Error("Parent must be wrapped inside PostsProvider");
  }

  return context;
};

export const BlogProvider = ({ children }) => {
  const [user, setUser] = useState();
  const [initialized, setInitialized] = useState(false);
  const [transactionPending, setTransactionPending] = useState(false);

  // const user = {
  //   name: "dog",
  //   avatar:
  //     "https://images.unsplash.com/photo-1517849845537-4d257902454a?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=435&q=80",
  //   title: "First blog",
  //   content: "welcome to my first post",
  // };

  const anchorWallet = useAnchorWallet();
  const { connection } = useConnection();
  const { publicKey } = useWallet();

  // console.log("public key", publicKey.toString());
  // console.log("connection", connection);
  // console.log("anchorWallet", anchorWallet);

  const program = useMemo(() => {
    if (anchorWallet) {
      const provider = new anchor.AnchorProvider(
        connection,
        anchorWallet,
        anchor.AnchorProvider.defaultOptions()
      );
      return new anchor.Program(idl, PROGRAM_KEY, provider);
    }
  }, [connection, anchorWallet]);

  // console.log("PROGRAM HERE", program);

  useEffect(() => {
    const start = async () => {
      console.log("starting app and fetching data");
      // IF there is a user FETCH POSTS
      //IF NO iser Set state to false (need a button to init user)
      if (program && publicKey) {
        try {
          setTransactionPending(true);

          //check if there is a user
          const [userPda] = await findProgramAddressSync(
            [utf8.encode("user"), publicKey.toBuffer()],
            program.programId
          );
          const user = await program.account.userAccount.fetch(userPda);
          if (user) {
            setInitialized(true);
          }
        } catch (error) {
          console.log("No User");
          setInitialized(false);
        } finally {
          setTransactionPending(false);
        }
      }
    };
    start();
  }, []);

  const initUser = async () => {
    if (program && publicKey) {
      try {
        setTransactionPending(true);
        const name = getRandomName();
        const avatar = getAvatarUrl(name);
        const [userPda] = await findProgramAddressSync(
          [utf8.encode("user"), publicKey.toBuffer()],
          program.programId
        );
        await program.methods
          .initUser(name, avatar)
          .accounts({
            userAccount: userPda,
            authority: publicKey,
            SystemProgram: SystemProgram.programId,
          })
          .rpc();
        setInitialized(true);
      } catch (error) {
        console.log(error);
      } finally {
        setTransactionPending(false);
      }
    }
  };

  return (
    <BlogContext.Provider
      value={{
        user,
        initialized,
        initUser,

      }}
    >
      {children}
    </BlogContext.Provider>
  );
};
